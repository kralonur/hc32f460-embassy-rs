#[doc = "Register `ERRCRH` reader"]
pub type R = crate::R<ErrcrhSpec>;
#[doc = "Register `ERRCRH` writer"]
pub type W = crate::W<ErrcrhSpec>;
#[doc = "Field `COMP8` reader - desc COMP8"]
pub type Comp8R = crate::BitReader;
#[doc = "Field `COMP8` writer - desc COMP8"]
pub type Comp8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEN` reader - desc COMPEN"]
pub type CompenR = crate::BitReader;
#[doc = "Field `COMPEN` writer - desc COMPEN"]
pub type CompenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc COMP8"]
    #[inline(always)]
    pub fn comp8(&self) -> Comp8R {
        Comp8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - desc COMPEN"]
    #[inline(always)]
    pub fn compen(&self) -> CompenR {
        CompenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc COMP8"]
    #[inline(always)]
    pub fn comp8(&mut self) -> Comp8W<'_, ErrcrhSpec> {
        Comp8W::new(self, 0)
    }
    #[doc = "Bit 7 - desc COMPEN"]
    #[inline(always)]
    pub fn compen(&mut self) -> CompenW<'_, ErrcrhSpec> {
        CompenW::new(self, 7)
    }
}
#[doc = "desc ERRCRH\n\nYou can [`read`](crate::Reg::read) this register and get [`errcrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errcrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrcrhSpec;
impl crate::RegisterSpec for ErrcrhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`errcrh::R`](R) reader structure"]
impl crate::Readable for ErrcrhSpec {}
#[doc = "`write(|w| ..)` method takes [`errcrh::W`](W) writer structure"]
impl crate::Writable for ErrcrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRCRH to value 0"]
impl crate::Resettable for ErrcrhSpec {}
