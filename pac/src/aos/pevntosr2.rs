#[doc = "Register `PEVNTOSR2` reader"]
pub type R = crate::R<Pevntosr2Spec>;
#[doc = "Register `PEVNTOSR2` writer"]
pub type W = crate::W<Pevntosr2Spec>;
#[doc = "Field `POS` reader - desc POS"]
pub type PosR = crate::FieldReader<u16>;
#[doc = "Field `POS` writer - desc POS"]
pub type PosW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc POS"]
    #[inline(always)]
    pub fn pos(&self) -> PosR {
        PosR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc POS"]
    #[inline(always)]
    pub fn pos(&mut self) -> PosW<'_, Pevntosr2Spec> {
        PosW::new(self, 0)
    }
}
#[doc = "desc PEVNTOSR2\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntosr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntosr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntosr2Spec;
impl crate::RegisterSpec for Pevntosr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntosr2::R`](R) reader structure"]
impl crate::Readable for Pevntosr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntosr2::W`](W) writer structure"]
impl crate::Writable for Pevntosr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTOSR2 to value 0"]
impl crate::Resettable for Pevntosr2Spec {}
