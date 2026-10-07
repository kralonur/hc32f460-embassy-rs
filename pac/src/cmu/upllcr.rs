#[doc = "Register `UPLLCR` reader"]
pub type R = crate::R<UpllcrSpec>;
#[doc = "Register `UPLLCR` writer"]
pub type W = crate::W<UpllcrSpec>;
#[doc = "Field `UPLLOFF` reader - desc UPLLOFF"]
pub type UplloffR = crate::BitReader;
#[doc = "Field `UPLLOFF` writer - desc UPLLOFF"]
pub type UplloffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc UPLLOFF"]
    #[inline(always)]
    pub fn uplloff(&self) -> UplloffR {
        UplloffR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc UPLLOFF"]
    #[inline(always)]
    pub fn uplloff(&mut self) -> UplloffW<'_, UpllcrSpec> {
        UplloffW::new(self, 0)
    }
}
#[doc = "desc UPLLCR\n\nYou can [`read`](crate::Reg::read) this register and get [`upllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpllcrSpec;
impl crate::RegisterSpec for UpllcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upllcr::R`](R) reader structure"]
impl crate::Readable for UpllcrSpec {}
#[doc = "`write(|w| ..)` method takes [`upllcr::W`](W) writer structure"]
impl crate::Writable for UpllcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPLLCR to value 0x01"]
impl crate::Resettable for UpllcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
