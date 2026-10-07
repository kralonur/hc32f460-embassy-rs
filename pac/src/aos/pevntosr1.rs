#[doc = "Register `PEVNTOSR1` reader"]
pub type R = crate::R<Pevntosr1Spec>;
#[doc = "Register `PEVNTOSR1` writer"]
pub type W = crate::W<Pevntosr1Spec>;
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
    pub fn pos(&mut self) -> PosW<'_, Pevntosr1Spec> {
        PosW::new(self, 0)
    }
}
#[doc = "desc PEVNTOSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`pevntosr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pevntosr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pevntosr1Spec;
impl crate::RegisterSpec for Pevntosr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pevntosr1::R`](R) reader structure"]
impl crate::Readable for Pevntosr1Spec {}
#[doc = "`write(|w| ..)` method takes [`pevntosr1::W`](W) writer structure"]
impl crate::Writable for Pevntosr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PEVNTOSR1 to value 0"]
impl crate::Resettable for Pevntosr1Spec {}
